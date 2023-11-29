; ModuleID = 'test_haybale.1d7024b5-cgu.0'
source_filename = "test_haybale.1d7024b5-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@vtable.0 = private constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h35eabced11febfffE", [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h597283028bc2a7c3E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2acd93dbd9a9effbE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2acd93dbd9a9effbE" }>, align 8, !dbg !0
@alloc_984a7eac172fbf403c24b8d0f53f1cd6 = private unnamed_addr constant <{ [15 x i8] }> <{ [15 x i8] c"test_haybale.rs" }>, align 1
@alloc_7da963ef0a7ed385b3d7c8a69b060f69 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_984a7eac172fbf403c24b8d0f53f1cd6, [16 x i8] c"\0F\00\00\00\00\00\00\00\09\00\00\00\09\00\00\00" }>, align 8
@str.1 = internal constant [28 x i8] c"attempt to add with overflow"
@alloc_3ac15f48dadb9efa0e647943db7daa93 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_984a7eac172fbf403c24b8d0f53f1cd6, [16 x i8] c"\0F\00\00\00\00\00\00\00\0A\00\00\00\09\00\00\00" }>, align 8
@str.2 = internal constant [33 x i8] c"attempt to multiply with overflow"
@alloc_f231c5475932f2a6468de0d432da65ce = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_984a7eac172fbf403c24b8d0f53f1cd6, [16 x i8] c"\0F\00\00\00\00\00\00\00\0B\00\00\00\11\00\00\00" }>, align 8
@__rustc_debug_gdb_scripts_section__ = linkonce_odr unnamed_addr constant [34 x i8] c"\01gdb_load_rust_pretty_printers.py\00", section ".debug_gdb_scripts", align 1

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17he1cbe36408b734f2E(ptr %f) unnamed_addr #0 !dbg !32 {
start:
  %f.dbg.spill = alloca ptr, align 8
  %0 = alloca {}, align 1
  %dummy.dbg.spill = alloca {}, align 1
  call void @llvm.dbg.declare(metadata ptr %dummy.dbg.spill, metadata !45, metadata !DIExpression()), !dbg !54
  call void @llvm.dbg.declare(metadata ptr %0, metadata !40, metadata !DIExpression()), !dbg !56
  store ptr %f, ptr %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %f.dbg.spill, metadata !39, metadata !DIExpression()), !dbg !57
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hac58fe89be63c2c9E(ptr %f), !dbg !58
  call void asm sideeffect "", "~{memory}"(), !dbg !59, !srcloc !60
  ret void, !dbg !61
}

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17h639e9182678e0bc5E(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #1 !dbg !62 {
start:
  %0 = alloca i64, align 8
  %sigpipe.dbg.spill = alloca i8, align 1
  %argv.dbg.spill = alloca ptr, align 8
  %argc.dbg.spill = alloca i64, align 8
  %main.dbg.spill = alloca ptr, align 8
  %_8 = alloca ptr, align 8
  %_5 = alloca i64, align 8
  store ptr %main, ptr %main.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %main.dbg.spill, metadata !71, metadata !DIExpression()), !dbg !77
  store i64 %argc, ptr %argc.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %argc.dbg.spill, metadata !72, metadata !DIExpression()), !dbg !78
  store ptr %argv, ptr %argv.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %argv.dbg.spill, metadata !73, metadata !DIExpression()), !dbg !79
  store i8 %sigpipe, ptr %sigpipe.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata ptr %sigpipe.dbg.spill, metadata !74, metadata !DIExpression()), !dbg !80
  store ptr %main, ptr %_8, align 8, !dbg !81
; call std::rt::lang_start_internal
  %1 = call i64 @_ZN3std2rt19lang_start_internal17h3d9b3b40accc3cf7E(ptr align 1 %_8, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe), !dbg !82
  store i64 %1, ptr %_5, align 8, !dbg !82
  %2 = load i64, ptr %_5, align 8, !dbg !83, !noundef !23
  store i64 %2, ptr %0, align 8, !dbg !83
  call void @llvm.dbg.declare(metadata ptr %0, metadata !75, metadata !DIExpression()), !dbg !84
  ret i64 %2, !dbg !85
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2acd93dbd9a9effbE"(ptr align 8 %_1) unnamed_addr #2 !dbg !86 {
start:
  %self.dbg.spill = alloca ptr, align 8
  %_1.dbg.spill = alloca ptr, align 8
  %self = alloca i8, align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !92, metadata !DIExpression(DW_OP_deref)), !dbg !93
  call void @llvm.dbg.declare(metadata ptr %self, metadata !94, metadata !DIExpression()), !dbg !112
  %_4 = load ptr, ptr %_1, align 8, !dbg !114, !nonnull !23, !noundef !23
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17he1cbe36408b734f2E(ptr %_4), !dbg !115
; call <() as std::process::Termination>::report
  %0 = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7e4b420ad7d9f74bE"(), !dbg !115
  store i8 %0, ptr %self, align 1, !dbg !115
  store ptr %self, ptr %self.dbg.spill, align 8, !dbg !116
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !117, metadata !DIExpression()), !dbg !125
  %_6 = load i8, ptr %self, align 1, !dbg !127, !noundef !23
  %1 = zext i8 %_6 to i32, !dbg !127
  ret i32 %1, !dbg !128
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h597283028bc2a7c3E"(ptr %_1) unnamed_addr #2 !dbg !129 {
start:
  %_1.dbg.spill = alloca ptr, align 8
  %_2 = alloca {}, align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !138, metadata !DIExpression()), !dbg !143
  call void @llvm.dbg.declare(metadata ptr %_2, metadata !139, metadata !DIExpression()), !dbg !143
  %0 = load ptr, ptr %_1, align 8, !dbg !143, !nonnull !23, !noundef !23
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17hcaa824a802960fa9E(ptr %0), !dbg !143
  ret i32 %1, !dbg !143
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17hac58fe89be63c2c9E(ptr %_1) unnamed_addr #2 !dbg !144 {
start:
  %_1.dbg.spill = alloca ptr, align 8
  %_2 = alloca {}, align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !146, metadata !DIExpression()), !dbg !150
  call void @llvm.dbg.declare(metadata ptr %_2, metadata !147, metadata !DIExpression()), !dbg !150
  call void %_1(), !dbg !150
  ret void, !dbg !150
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17hcaa824a802960fa9E(ptr %0) unnamed_addr #2 personality ptr @rust_eh_personality !dbg !151 {
start:
  %1 = alloca { ptr, i32 }, align 8
  %_2 = alloca {}, align 1
  %_1 = alloca ptr, align 8
  store ptr %0, ptr %_1, align 8
  call void @llvm.dbg.declare(metadata ptr %_1, metadata !155, metadata !DIExpression()), !dbg !157
  call void @llvm.dbg.declare(metadata ptr %_2, metadata !156, metadata !DIExpression()), !dbg !157
; invoke std::rt::lang_start::{{closure}}
  %2 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2acd93dbd9a9effbE"(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup, !dbg !157

bb3:                                              ; preds = %cleanup
  %3 = load ptr, ptr %1, align 8, !dbg !157, !noundef !23
  %4 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1, !dbg !157
  %5 = load i32, ptr %4, align 8, !dbg !157, !noundef !23
  %6 = insertvalue { ptr, i32 } poison, ptr %3, 0, !dbg !157
  %7 = insertvalue { ptr, i32 } %6, i32 %5, 1, !dbg !157
  resume { ptr, i32 } %7, !dbg !157

cleanup:                                          ; preds = %start
  %8 = landingpad { ptr, i32 }
          cleanup
  %9 = extractvalue { ptr, i32 } %8, 0
  %10 = extractvalue { ptr, i32 } %8, 1
  %11 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %9, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %10, ptr %12, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %2, !dbg !157
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h35eabced11febfffE"(ptr %_1) unnamed_addr #2 !dbg !158 {
start:
  %_1.dbg.spill = alloca ptr, align 8
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !164, metadata !DIExpression()), !dbg !167
  ret void, !dbg !167
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7e4b420ad7d9f74bE"() unnamed_addr #2 !dbg !168 {
start:
  %_1.dbg.spill = alloca {}, align 1
  %self.dbg.spill = alloca {}, align 1
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !173, metadata !DIExpression()), !dbg !175
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !174, metadata !DIExpression()), !dbg !175
  ret i8 0, !dbg !176
}

; test_haybale::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN12test_haybale4main17hd752ef4e448a2d30E() unnamed_addr #1 !dbg !177 {
start:
  %y.dbg.spill = alloca i32, align 4
  %x.dbg.spill = alloca i32, align 4
  store i32 5, ptr %x.dbg.spill, align 4, !dbg !185
  call void @llvm.dbg.declare(metadata ptr %x.dbg.spill, metadata !181, metadata !DIExpression()), !dbg !186
  store i32 18, ptr %y.dbg.spill, align 4, !dbg !187
  call void @llvm.dbg.declare(metadata ptr %y.dbg.spill, metadata !183, metadata !DIExpression()), !dbg !188
; call test_haybale::s_algebra
  %_3 = call i32 @_ZN12test_haybale9s_algebra17h43af51aac41876bbE(i32 5, i32 18), !dbg !189
  ret void, !dbg !190
}

; test_haybale::s_algebra
; Function Attrs: nonlazybind uwtable
define internal i32 @_ZN12test_haybale9s_algebra17h43af51aac41876bbE(i32 %0, i32 %1) unnamed_addr #1 !dbg !191 {
start:
  %2 = alloca i32, align 4
  %y = alloca i32, align 4
  %x = alloca i32, align 4
  store i32 %0, ptr %x, align 4
  store i32 %1, ptr %y, align 4
  call void @llvm.dbg.declare(metadata ptr %x, metadata !195, metadata !DIExpression()), !dbg !199
  call void @llvm.dbg.declare(metadata ptr %y, metadata !196, metadata !DIExpression()), !dbg !200
  %_3 = load i32, ptr %y, align 4, !dbg !201, !noundef !23
  %3 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %_3, i32 4), !dbg !201
  %_4.0 = extractvalue { i32, i1 } %3, 0, !dbg !201
  %_4.1 = extractvalue { i32, i1 } %3, 1, !dbg !201
  %4 = call i1 @llvm.expect.i1(i1 %_4.1, i1 false), !dbg !201
  br i1 %4, label %panic, label %bb1, !dbg !201

bb1:                                              ; preds = %start
  store i32 %_4.0, ptr %x, align 4, !dbg !202
  %_5 = load i32, ptr %x, align 4, !dbg !203, !noundef !23
  %5 = call { i32, i1 } @llvm.smul.with.overflow.i32(i32 2, i32 %_5), !dbg !204
  %_6.0 = extractvalue { i32, i1 } %5, 0, !dbg !204
  %_6.1 = extractvalue { i32, i1 } %5, 1, !dbg !204
  %6 = call i1 @llvm.expect.i1(i1 %_6.1, i1 false), !dbg !204
  br i1 %6, label %panic1, label %bb2, !dbg !204

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hd3becb37bf39c010E(ptr align 1 @str.1, i64 28, ptr align 8 @alloc_7da963ef0a7ed385b3d7c8a69b060f69) #7, !dbg !201
  unreachable, !dbg !201

bb2:                                              ; preds = %bb1
  store i32 %_6.0, ptr %y, align 4, !dbg !205
  %_8 = load i32, ptr %x, align 4, !dbg !206, !noundef !23
  %7 = call { i32, i1 } @llvm.smul.with.overflow.i32(i32 %_8, i32 4), !dbg !207
  %_9.0 = extractvalue { i32, i1 } %7, 0, !dbg !207
  %_9.1 = extractvalue { i32, i1 } %7, 1, !dbg !207
  %8 = call i1 @llvm.expect.i1(i1 %_9.1, i1 false), !dbg !207
  br i1 %8, label %panic2, label %bb3, !dbg !207

panic1:                                           ; preds = %bb1
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hd3becb37bf39c010E(ptr align 1 @str.2, i64 33, ptr align 8 @alloc_3ac15f48dadb9efa0e647943db7daa93) #7, !dbg !204
  unreachable, !dbg !204

bb3:                                              ; preds = %bb2
  %_10 = load i32, ptr %y, align 4, !dbg !208, !noundef !23
  %9 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %_9.0, i32 %_10), !dbg !207
  %_11.0 = extractvalue { i32, i1 } %9, 0, !dbg !207
  %_11.1 = extractvalue { i32, i1 } %9, 1, !dbg !207
  %10 = call i1 @llvm.expect.i1(i1 %_11.1, i1 false), !dbg !207
  br i1 %10, label %panic3, label %bb4, !dbg !207

panic2:                                           ; preds = %bb2
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hd3becb37bf39c010E(ptr align 1 @str.2, i64 33, ptr align 8 @alloc_f231c5475932f2a6468de0d432da65ce) #7, !dbg !207
  unreachable, !dbg !207

bb4:                                              ; preds = %bb3
  store i32 %_11.0, ptr %2, align 4, !dbg !207
  call void @llvm.dbg.declare(metadata ptr %2, metadata !197, metadata !DIExpression()), !dbg !209
  ret i32 %_11.0, !dbg !210

panic3:                                           ; preds = %bb3
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hd3becb37bf39c010E(ptr align 1 @str.1, i64 28, ptr align 8 @alloc_f231c5475932f2a6468de0d432da65ce) #7, !dbg !207
  unreachable, !dbg !207
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare void @llvm.dbg.declare(metadata, metadata, metadata) #3

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h3d9b3b40accc3cf7E(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #1

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #3

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #4

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17hd3becb37bf39c010E(ptr align 1, i64, ptr align 8) unnamed_addr #5

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.smul.with.overflow.i32(i32, i32) #3

; Function Attrs: nonlazybind
define i32 @main(i32 %0, ptr %1) unnamed_addr #6 {
top:
  %2 = load volatile i8, ptr @__rustc_debug_gdb_scripts_section__, align 1
  %3 = sext i32 %0 to i64
; call std::rt::lang_start
  %4 = call i64 @_ZN3std2rt10lang_start17h639e9182678e0bc5E(ptr @_ZN12test_haybale4main17hd752ef4e448a2d30E, i64 %3, ptr %1, i8 0)
  %5 = trunc i64 %4 to i32
  ret i32 %5
}

attributes #0 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #4 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #5 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #6 = { nonlazybind "target-cpu"="x86-64" }
attributes #7 = { noreturn }

!llvm.module.flags = !{!24, !25, !26, !27, !28}
!llvm.dbg.cu = !{!29}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable}", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable_type}", file: !2, size: 384, align: 64, flags: DIFlagArtificial, elements: !4, vtableHolder: !14, templateParams: !23, identifier: "98894a22458c78cffa5451a638630ec")
!4 = !{!5, !8, !10, !11, !12, !13}
!5 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !3, file: !2, baseType: !6, size: 64, align: 64)
!6 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const ()", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 0)
!7 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!8 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !3, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!9 = !DIBasicType(name: "usize", size: 64, encoding: DW_ATE_unsigned)
!10 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !3, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!11 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !3, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!12 = !DIDerivedType(tag: DW_TAG_member, name: "__method4", scope: !3, file: !2, baseType: !6, size: 64, align: 64, offset: 256)
!13 = !DIDerivedType(tag: DW_TAG_member, name: "__method5", scope: !3, file: !2, baseType: !6, size: 64, align: 64, offset: 320)
!14 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<()>", scope: !15, file: !2, size: 64, align: 64, elements: !18, templateParams: !23, identifier: "ac66f759e096f507fe53ea93fbe27875")
!15 = !DINamespace(name: "lang_start", scope: !16)
!16 = !DINamespace(name: "rt", scope: !17)
!17 = !DINamespace(name: "std", scope: null)
!18 = !{!19}
!19 = !DIDerivedType(tag: DW_TAG_member, name: "main", scope: !14, file: !2, baseType: !20, size: 64, align: 64)
!20 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn()", baseType: !21, size: 64, align: 64, dwarfAddressSpace: 0)
!21 = !DISubroutineType(types: !22)
!22 = !{null}
!23 = !{}
!24 = !{i32 8, !"PIC Level", i32 2}
!25 = !{i32 7, !"PIE Level", i32 2}
!26 = !{i32 2, !"RtLibUseGOT", i32 1}
!27 = !{i32 2, !"Dwarf Version", i32 4}
!28 = !{i32 2, !"Debug Info Version", i32 3}
!29 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !30, producer: "clang LLVM (rustc version 1.70.0 (90c541806 2023-05-31) (built from a source tarball))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !31, splitDebugInlining: false)
!30 = !DIFile(filename: "test_haybale.rs/@/test_haybale.1d7024b5-cgu.0", directory: "/mnt/c/Users/catne/Desktop/school/703/test_code")
!31 = !{!0}
!32 = distinct !DISubprogram(name: "__rust_begin_short_backtrace<fn(), ()>", linkageName: "_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17he1cbe36408b734f2E", scope: !34, file: !33, line: 130, type: !36, scopeLine: 130, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !42, retainedNodes: !38)
!33 = !DIFile(filename: "/build/rustc-wAuwbs/rustc-1.70.0+dfsg0ubuntu1~bpo2/library/std/src/sys_common/backtrace.rs", directory: "", checksumkind: CSK_MD5, checksum: "0460e8805fbe2f82577251a7dab64163")
!34 = !DINamespace(name: "backtrace", scope: !35)
!35 = !DINamespace(name: "sys_common", scope: !17)
!36 = !DISubroutineType(types: !37)
!37 = !{null, !20}
!38 = !{!39, !40}
!39 = !DILocalVariable(name: "f", arg: 1, scope: !32, file: !33, line: 130, type: !20)
!40 = !DILocalVariable(name: "result", scope: !41, file: !33, line: 134, type: !7, align: 1)
!41 = distinct !DILexicalBlock(scope: !32, file: !33, line: 134, column: 5)
!42 = !{!43, !44}
!43 = !DITemplateTypeParameter(name: "F", type: !20)
!44 = !DITemplateTypeParameter(name: "T", type: !7)
!45 = !DILocalVariable(name: "dummy", scope: !46, file: !47, line: 295, type: !7, align: 1)
!46 = distinct !DISubprogram(name: "black_box<()>", linkageName: "_ZN4core4hint9black_box17h2c6da4a5bab7a016E", scope: !48, file: !47, line: 295, type: !50, scopeLine: 295, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !53, retainedNodes: !52)
!47 = !DIFile(filename: "/build/rustc-wAuwbs/rustc-1.70.0+dfsg0ubuntu1~bpo2/library/core/src/hint.rs", directory: "", checksumkind: CSK_MD5, checksum: "bdccd67e2121209a433721fb94464813")
!48 = !DINamespace(name: "hint", scope: !49)
!49 = !DINamespace(name: "core", scope: null)
!50 = !DISubroutineType(types: !51)
!51 = !{null, !7}
!52 = !{!45}
!53 = !{!44}
!54 = !DILocation(line: 295, column: 27, scope: !46, inlinedAt: !55)
!55 = !DILocation(line: 137, column: 5, scope: !41)
!56 = !DILocation(line: 134, column: 9, scope: !41)
!57 = !DILocation(line: 130, column: 43, scope: !32)
!58 = !DILocation(line: 134, column: 18, scope: !32)
!59 = !DILocation(line: 296, column: 5, scope: !46, inlinedAt: !55)
!60 = !{i32 1299228}
!61 = !DILocation(line: 140, column: 2, scope: !32)
!62 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_ZN3std2rt10lang_start17h639e9182678e0bc5E", scope: !16, file: !63, line: 159, type: !64, scopeLine: 159, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !53, retainedNodes: !70)
!63 = !DIFile(filename: "/build/rustc-wAuwbs/rustc-1.70.0+dfsg0ubuntu1~bpo2/library/std/src/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "cc84fb732e98226bd431c2a38372d049")
!64 = !DISubroutineType(types: !65)
!65 = !{!66, !20, !66, !67, !69}
!66 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!67 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !68, size: 64, align: 64, dwarfAddressSpace: 0)
!68 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !69, size: 64, align: 64, dwarfAddressSpace: 0)
!69 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!70 = !{!71, !72, !73, !74, !75}
!71 = !DILocalVariable(name: "main", arg: 1, scope: !62, file: !63, line: 160, type: !20)
!72 = !DILocalVariable(name: "argc", arg: 2, scope: !62, file: !63, line: 161, type: !66)
!73 = !DILocalVariable(name: "argv", arg: 3, scope: !62, file: !63, line: 162, type: !67)
!74 = !DILocalVariable(name: "sigpipe", arg: 4, scope: !62, file: !63, line: 163, type: !69)
!75 = !DILocalVariable(name: "v", scope: !76, file: !63, line: 165, type: !66, align: 8)
!76 = distinct !DILexicalBlock(scope: !62, file: !63, line: 165, column: 5)
!77 = !DILocation(line: 160, column: 5, scope: !62)
!78 = !DILocation(line: 161, column: 5, scope: !62)
!79 = !DILocation(line: 162, column: 5, scope: !62)
!80 = !DILocation(line: 163, column: 5, scope: !62)
!81 = !DILocation(line: 166, column: 10, scope: !62)
!82 = !DILocation(line: 165, column: 17, scope: !62)
!83 = !DILocation(line: 165, column: 12, scope: !62)
!84 = !DILocation(line: 165, column: 12, scope: !76)
!85 = !DILocation(line: 172, column: 2, scope: !62)
!86 = distinct !DISubprogram(name: "{closure#0}<()>", linkageName: "_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2acd93dbd9a9effbE", scope: !15, file: !63, line: 166, type: !87, scopeLine: 166, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !53, retainedNodes: !91)
!87 = !DISubroutineType(types: !88)
!88 = !{!89, !90}
!89 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!90 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!91 = !{!92}
!92 = !DILocalVariable(name: "main", scope: !86, file: !63, line: 160, type: !20, align: 8)
!93 = !DILocation(line: 160, column: 5, scope: !86)
!94 = !DILocalVariable(name: "self", arg: 1, scope: !95, file: !96, line: 1873, type: !97)
!95 = distinct !DISubprogram(name: "to_i32", linkageName: "_ZN3std7process8ExitCode6to_i3217hefd9016a1bf3d6d8E", scope: !97, file: !96, line: 1873, type: !108, scopeLine: 1873, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !23, declaration: !110, retainedNodes: !111)
!96 = !DIFile(filename: "/build/rustc-wAuwbs/rustc-1.70.0+dfsg0ubuntu1~bpo2/library/std/src/process.rs", directory: "", checksumkind: CSK_MD5, checksum: "fca842fd83a95a3beaf417a4526afa5b")
!97 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !98, file: !2, size: 8, align: 8, elements: !99, templateParams: !23, identifier: "9219a7a59ae876a53ccd0db1f1dc8ba3")
!98 = !DINamespace(name: "process", scope: !17)
!99 = !{!100}
!100 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !97, file: !2, baseType: !101, size: 8, align: 8)
!101 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !102, file: !2, size: 8, align: 8, elements: !106, templateParams: !23, identifier: "9635d21a0b8833213efdb2ca2fcb7155")
!102 = !DINamespace(name: "process_common", scope: !103)
!103 = !DINamespace(name: "process", scope: !104)
!104 = !DINamespace(name: "unix", scope: !105)
!105 = !DINamespace(name: "sys", scope: !17)
!106 = !{!107}
!107 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !101, file: !2, baseType: !69, size: 8, align: 8)
!108 = !DISubroutineType(types: !109)
!109 = !{!89, !97}
!110 = !DISubprogram(name: "to_i32", linkageName: "_ZN3std7process8ExitCode6to_i3217hefd9016a1bf3d6d8E", scope: !97, file: !96, line: 1873, type: !108, scopeLine: 1873, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!111 = !{!94}
!112 = !DILocation(line: 1873, column: 19, scope: !95, inlinedAt: !113)
!113 = !DILocation(line: 166, column: 92, scope: !86)
!114 = !DILocation(line: 166, column: 77, scope: !86)
!115 = !DILocation(line: 166, column: 18, scope: !86)
!116 = !DILocation(line: 1874, column: 9, scope: !95, inlinedAt: !113)
!117 = !DILocalVariable(name: "self", arg: 1, scope: !118, file: !119, line: 593, type: !122)
!118 = distinct !DISubprogram(name: "as_i32", linkageName: "_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217ha5d8522eab222aecE", scope: !101, file: !119, line: 593, type: !120, scopeLine: 593, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !23, declaration: !123, retainedNodes: !124)
!119 = !DIFile(filename: "/build/rustc-wAuwbs/rustc-1.70.0+dfsg0ubuntu1~bpo2/library/std/src/sys/unix/process/process_common.rs", directory: "", checksumkind: CSK_MD5, checksum: "9bd71fcd1bd04bc8d33b8ff713a403c8")
!120 = !DISubroutineType(types: !121)
!121 = !{!89, !122}
!122 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::sys::unix::process::process_common::ExitCode", baseType: !101, size: 64, align: 64, dwarfAddressSpace: 0)
!123 = !DISubprogram(name: "as_i32", linkageName: "_ZN3std3sys4unix7process14process_common8ExitCode6as_i3217ha5d8522eab222aecE", scope: !101, file: !119, line: 593, type: !120, scopeLine: 593, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!124 = !{!117}
!125 = !DILocation(line: 593, column: 19, scope: !118, inlinedAt: !126)
!126 = !DILocation(line: 1874, column: 16, scope: !95, inlinedAt: !113)
!127 = !DILocation(line: 594, column: 9, scope: !118, inlinedAt: !126)
!128 = !DILocation(line: 166, column: 100, scope: !86)
!129 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h597283028bc2a7c3E", scope: !131, file: !130, line: 250, type: !134, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !140, retainedNodes: !137)
!130 = !DIFile(filename: "/build/rustc-wAuwbs/rustc-1.70.0+dfsg0ubuntu1~bpo2/library/core/src/ops/function.rs", directory: "", checksumkind: CSK_MD5, checksum: "56fb008eac3df8d06ce524ffb023f0b6")
!131 = !DINamespace(name: "FnOnce", scope: !132)
!132 = !DINamespace(name: "function", scope: !133)
!133 = !DINamespace(name: "ops", scope: !49)
!134 = !DISubroutineType(types: !135)
!135 = !{!89, !136}
!136 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!137 = !{!138, !139}
!138 = !DILocalVariable(arg: 1, scope: !129, file: !130, line: 250, type: !136)
!139 = !DILocalVariable(arg: 2, scope: !129, file: !130, line: 250, type: !7)
!140 = !{!141, !142}
!141 = !DITemplateTypeParameter(name: "Self", type: !14)
!142 = !DITemplateTypeParameter(name: "Args", type: !7)
!143 = !DILocation(line: 250, column: 5, scope: !129)
!144 = distinct !DISubprogram(name: "call_once<fn(), ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17hac58fe89be63c2c9E", scope: !131, file: !130, line: 250, type: !36, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !148, retainedNodes: !145)
!145 = !{!146, !147}
!146 = !DILocalVariable(arg: 1, scope: !144, file: !130, line: 250, type: !20)
!147 = !DILocalVariable(arg: 2, scope: !144, file: !130, line: 250, type: !7)
!148 = !{!149, !142}
!149 = !DITemplateTypeParameter(name: "Self", type: !20)
!150 = !DILocation(line: 250, column: 5, scope: !144)
!151 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17hcaa824a802960fa9E", scope: !131, file: !130, line: 250, type: !152, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !140, retainedNodes: !154)
!152 = !DISubroutineType(types: !153)
!153 = !{!89, !14}
!154 = !{!155, !156}
!155 = !DILocalVariable(arg: 1, scope: !151, file: !130, line: 250, type: !14)
!156 = !DILocalVariable(arg: 2, scope: !151, file: !130, line: 250, type: !7)
!157 = !DILocation(line: 250, column: 5, scope: !151)
!158 = distinct !DISubprogram(name: "drop_in_place<std::rt::lang_start::{closure_env#0}<()>>", linkageName: "_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h35eabced11febfffE", scope: !160, file: !159, line: 490, type: !161, scopeLine: 490, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !165, retainedNodes: !163)
!159 = !DIFile(filename: "/build/rustc-wAuwbs/rustc-1.70.0+dfsg0ubuntu1~bpo2/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "4b0ac29df94a7dc1bf2bc7efca5e253a")
!160 = !DINamespace(name: "ptr", scope: !49)
!161 = !DISubroutineType(types: !162)
!162 = !{null, !136}
!163 = !{!164}
!164 = !DILocalVariable(arg: 1, scope: !158, file: !159, line: 490, type: !136)
!165 = !{!166}
!166 = !DITemplateTypeParameter(name: "T", type: !14)
!167 = !DILocation(line: 490, column: 1, scope: !158)
!168 = distinct !DISubprogram(name: "report", linkageName: "_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7e4b420ad7d9f74bE", scope: !169, file: !96, line: 2236, type: !170, scopeLine: 2236, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !23, retainedNodes: !172)
!169 = !DINamespace(name: "{impl#53}", scope: !98)
!170 = !DISubroutineType(types: !171)
!171 = !{!97, !7}
!172 = !{!173, !174}
!173 = !DILocalVariable(name: "self", scope: !168, file: !96, line: 2236, type: !7, align: 1)
!174 = !DILocalVariable(arg: 1, scope: !168, file: !96, line: 2236, type: !7)
!175 = !DILocation(line: 2236, column: 15, scope: !168)
!176 = !DILocation(line: 2238, column: 6, scope: !168)
!177 = distinct !DISubprogram(name: "main", linkageName: "_ZN12test_haybale4main17hd752ef4e448a2d30E", scope: !179, file: !178, line: 1, type: !21, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagMainSubprogram, unit: !29, templateParams: !23, retainedNodes: !180)
!178 = !DIFile(filename: "test_haybale.rs", directory: "/mnt/c/Users/catne/Desktop/school/703/test_code", checksumkind: CSK_MD5, checksum: "64d7c2f432b26f036938d6fe762cc885")
!179 = !DINamespace(name: "test_haybale", scope: null)
!180 = !{!181, !183}
!181 = !DILocalVariable(name: "x", scope: !182, file: !178, line: 2, type: !89, align: 4)
!182 = distinct !DILexicalBlock(scope: !177, file: !178, line: 2, column: 5)
!183 = !DILocalVariable(name: "y", scope: !184, file: !178, line: 3, type: !89, align: 4)
!184 = distinct !DILexicalBlock(scope: !182, file: !178, line: 3, column: 5)
!185 = !DILocation(line: 2, column: 17, scope: !177)
!186 = !DILocation(line: 2, column: 9, scope: !182)
!187 = !DILocation(line: 3, column: 17, scope: !182)
!188 = !DILocation(line: 3, column: 9, scope: !184)
!189 = !DILocation(line: 5, column: 5, scope: !184)
!190 = !DILocation(line: 6, column: 2, scope: !177)
!191 = distinct !DISubprogram(name: "s_algebra", linkageName: "_ZN12test_haybale9s_algebra17h43af51aac41876bbE", scope: !179, file: !178, line: 8, type: !192, scopeLine: 8, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !23, retainedNodes: !194)
!192 = !DISubroutineType(types: !193)
!193 = !{!89, !89, !89}
!194 = !{!195, !196, !197}
!195 = !DILocalVariable(name: "x", arg: 1, scope: !191, file: !178, line: 8, type: !89)
!196 = !DILocalVariable(name: "y", arg: 2, scope: !191, file: !178, line: 8, type: !89)
!197 = !DILocalVariable(name: "w", scope: !198, file: !178, line: 11, type: !89, align: 4)
!198 = distinct !DILexicalBlock(scope: !191, file: !178, line: 11, column: 5)
!199 = !DILocation(line: 8, column: 14, scope: !191)
!200 = !DILocation(line: 8, column: 25, scope: !191)
!201 = !DILocation(line: 9, column: 9, scope: !191)
!202 = !DILocation(line: 9, column: 5, scope: !191)
!203 = !DILocation(line: 10, column: 11, scope: !191)
!204 = !DILocation(line: 10, column: 9, scope: !191)
!205 = !DILocation(line: 10, column: 5, scope: !191)
!206 = !DILocation(line: 11, column: 18, scope: !191)
!207 = !DILocation(line: 11, column: 17, scope: !191)
!208 = !DILocation(line: 11, column: 25, scope: !191)
!209 = !DILocation(line: 11, column: 9, scope: !198)
!210 = !DILocation(line: 14, column: 2, scope: !191)
