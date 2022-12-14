use std::env;
use std::ffi::OsString;
use std::fs::{create_dir_all, read_dir, File};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    /* === Building for embedded === */
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");

    /* === Linking to microbit libs === */
    // let arm_eabi_lib_dir = PathBuf::from(
    //     env::var_os("ARM_EABI_LIB_DIR")
    //         .unwrap_or_else(|| OsString::from_str("/usr/arm-none-eabi/lib").unwrap()),
    // )
    // .canonicalize()
    // .unwrap();
    let includes_dir = PathBuf::from("./includes");
    let prebuilt_dir = PathBuf::from("./prebuilts");

    if !includes_dir.exists() {
        create_dir_all(&includes_dir).unwrap();
    }
    let includes_dir = includes_dir.canonicalize().unwrap();
    if !prebuilt_dir.exists() {
        create_dir_all(&prebuilt_dir).unwrap();
    }
    let prebuilt_dir = prebuilt_dir.canonicalize().unwrap();

    #[cfg(not(feature = "prebuilt"))]
    {
        let microbit_path = PathBuf::from("./external/microbit").canonicalize().unwrap();
        let header = microbit_path.join("inc").join("MicroBit.h");
        let yotta_modules_path = microbit_path.join("yotta_modules");
        let microbit_dal_headers = yotta_modules_path.join("microbit-dal").join("inc");
        let build_dir = microbit_path.join("build").join("bbc-microbit-classic-gcc");
        let static_libs = [
            build_dir.join("source").join("microbit.a"),
            build_dir
                .join("ym")
                .join("ble")
                .join("source")
                .join("ble.a"),
            build_dir
                .join("ym")
                .join("ble-nrf51822")
                .join("source")
                .join("ble-nrf51822.a"),
            build_dir
                .join("ym")
                .join("mbed-classic")
                .join("existing")
                .join("mbed-classic.a"),
            build_dir
                .join("ym")
                .join("microbit-dal")
                .join("source")
                .join("microbit-dal.a"),
            build_dir
                .join("ym")
                .join("nrf51-sdk")
                .join("source")
                .join("nrf51-sdk.a"),
        ];
        println!("cargo:rerun-if-changed={}", header.to_str().unwrap());
        for lib in static_libs.iter() {
            println!("cargo:rerun-if-changed={}", lib.to_str().unwrap());
        }

        let headers = vec![
            header,
            microbit_dal_headers.join("types").join("ManagedString.h"),
        ];

        if let Err(output) = std::process::Command::new("yotta")
            .args(["build"])
            .current_dir(&microbit_path)
            .spawn()
            .unwrap()
            .wait_with_output()
        {
            panic!("Error building microbit: {:?}", output);
        };

        for header in headers.iter() {
            std::fs::copy(header, includes_dir.join(header.file_name().unwrap())).unwrap();
        }
        for lib in static_libs.iter() {
            std::fs::copy(
                lib,
                prebuilt_dir
                    .join(["lib", lib.file_name().and_then(|f| f.to_str()).unwrap()].concat()),
            )
            .unwrap();
        }
    }
    #[cfg(feature = "prebuilt")]
    {
        if !includes_dir.join("MicroBit.h").exists() || !prebuilt_dir.join("libmicrobit.a").exists()
        {
            panic!("Please build without the feature `prebuilt` to compile MicroBit from source (prebuilts not found)");
        }
        println!(
            "cargo:rerun-if-changed={}",
            includes_dir.join("MicroBit.h").to_str().unwrap()
        );
        println!(
            "cargo:rerun-if-changed={}",
            prebuilt_dir.join("libmicrobit.a").to_str().unwrap()
        );
    }

    // NOTE: Link to required C++ deps
    // for lib in ["m"] {
    //     println!("cargo:rustc-link-lib={lib}");
    // }

    // println!(
    //     "cargo:rustc-link-search=all={}",
    //     arm_eabi_lib_dir.to_str().unwrap()
    // );
    println!(
        "cargo:rustc-link-search=native={}",
        prebuilt_dir.to_str().unwrap()
    );

    // NOTE: Link to prebuilts libraries
    for lib in read_dir(prebuilt_dir).unwrap().flatten() {
        println!(
            "cargo:rustc-link-lib=static={}",
            lib.file_name()
                .to_str()
                .unwrap()
                .strip_suffix(".a")
                .and_then(|f| f.strip_prefix("lib"))
                .unwrap()
        );
    }
}

/* Yotta build params
arm-none-eabi-g++ -DNRF51 -DTARGET_NORDIC -DTARGET_M0 -D__MBED__=1 -DMCU_NORDIC_16K \
    -DTARGET_NRF51_MICROBIT -DTARGET_MCU_NORDIC_16K -DTARGET_MCU_NRF51_16K_S110  \
    -DTARGET_NRF_LFCLK_RC -DTARGET_MCU_NORDIC_16K -D__CORTEX_M0 -DARM_MATH_CM0 \
    -DYOTTA_MODULE_NAME=mbed-classic \
    -Iexternal/microbit/build/bbc-microbit-classic-gcc/generated/include \
    -Iexternal/microbit \
    -Iexternal/microbit/yotta_modules/microbit-dal \
    -Iexternal/microbit/yotta_modules/mbed-classic \
    -Iexternal/microbit/yotta_modules/ble \
    -Iexternal/microbit/yotta_modules/ble-nrf51822 \
    -Iexternal/microbit/yotta_modules/nrf51-sdk \
    -Iexternal/microbit/yotta_modules/mbed-classic/api \
    -Iexternal/microbit/yotta_modules/mbed-classic/hal \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/cmsis \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/cmsis/TARGET_NORDIC \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/cmsis/TARGET_NORDIC/TARGET_MCU_NRF51822 \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/cmsis/TARGET_NORDIC/TARGET_MCU_NRF51822/TOOLCHAIN_GCC_ARM \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/cmsis/TARGET_NORDIC/TARGET_MCU_NRF51822/TOOLCHAIN_GCC_ARM/TARGET_MCU_NRF51_16K_S110 \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC/TARGET_MCU_NRF51822 \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC/TARGET_MCU_NRF51822/Lib \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC/TARGET_MCU_NRF51822/Lib/nordic_sdk \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC/TARGET_MCU_NRF51822/Lib/nordic_sdk/components \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC/TARGET_MCU_NRF51822/Lib/nordic_sdk/components/libraries \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC/TARGET_MCU_NRF51822/Lib/nordic_sdk/components/libraries/crc16 \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC/TARGET_MCU_NRF51822/Lib/nordic_sdk/components/libraries/scheduler \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC/TARGET_MCU_NRF51822/Lib/nordic_sdk/components/libraries/util \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC/TARGET_MCU_NRF51822/Lib/s110_nrf51822_8_0_0 \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC/TARGET_MCU_NRF51822/Lib/s130_nrf51822_1_0_0 \
    -Iexternal/microbit/yotta_modules/mbed-classic/targets/hal/TARGET_NORDIC/TARGET_MCU_NRF51822/TARGET_NRF51_MICROBIT \
    -fno-exceptions -fno-unwind-tables -ffunction-sections -fdata-sections -Wall -Wextra -fno-rtti -fno-threadsafe-statics \
    -mcpu=cortex-m0 -mthumb -D__thumb2__ -std=c++11 -fwrapv -Os -g -gdwarf-3 -DNDEBUG   \
    -DTOOLCHAIN_GCC -DTOOLCHAIN_GCC_ARM -DMBED_OPERATORS \
    -include "external/microbit/build/bbc-microbit-classic-gcc/yotta_config.h" \
    -w -MMD -MT ym/mbed-classic/existing/CMakeFiles/mbed-classic.dir/common/FileSystemLike.cpp.o \
    -MF DEPFILE -o ym/mbed-classic/existing/CMakeFiles/mbed-classic.dir/common/FileSystemLike.cpp.o \
    -c /home/kuro/Cours/inge.courses-s7/apri/pfp/external/microbit/yotta_modules/mbed-classic/common/FileSystemLike.cpp


    -fno-exceptions -fno-unwind-tables -Wl,--gc-sections -Wl,--sort-common -Wl,--sort-section=alignment -Wl,-wrap,main -crash -mcpu=cortex-m0 -mthumb -T"/home/kuro/Cours/inge.courses-s7/apri/pfp/external/microbit/yotta_targets/bbc-microbit-classic-gcc/CMake/../ld/NRF51822.ld" -Wl,-Map,cmTC_ac11e.map -Wl,--start-group CMakeFiles/cmTC_ac11e.dir/testCCompiler.c.o    -lm -lc -lgcc -lm -lc -lgcc -Wl,--end-group  --specs=nano.specs
*/
