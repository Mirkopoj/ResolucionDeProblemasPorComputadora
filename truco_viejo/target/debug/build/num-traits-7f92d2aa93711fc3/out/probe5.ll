; ModuleID = 'probe5.d76e8ea20a2eba94-cgu.0'
source_filename = "probe5.d76e8ea20a2eba94-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@alloc_011dc74699911af6a9c0bfc0e7554e82 = private unnamed_addr constant <{ [77 x i8] }> <{ [77 x i8] c"/rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library/core/src/ops/arith.rs" }>, align 1
@alloc_9c1fe1ff2e3925050b4dd61043010002 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_011dc74699911af6a9c0bfc0e7554e82, [16 x i8] c"M\00\00\00\00\00\00\00\F8\02\00\00\01\00\00\00" }>, align 8
@str.0 = internal constant [28 x i8] c"attempt to add with overflow"
@alloc_2e38410fced2c310c68bdf2d45d0c3bd = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"\02\00\00\00" }>, align 4

; <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN66_$LT$i32$u20$as$u20$core..ops..arith..AddAssign$LT$$RF$i32$GT$$GT$10add_assign17h808330a3e4e24bd3E"(ptr align 4 %self, ptr align 4 %other) unnamed_addr #0 {
start:
  %other1 = load i32, ptr %other, align 4, !noundef !3
  %0 = load i32, ptr %self, align 4, !noundef !3
  %1 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %0, i32 %other1)
  %_4.0 = extractvalue { i32, i1 } %1, 0
  %_4.1 = extractvalue { i32, i1 } %1, 1
  %2 = call i1 @llvm.expect.i1(i1 %_4.1, i1 false)
  br i1 %2, label %panic, label %bb1

bb1:                                              ; preds = %start
  store i32 %_4.0, ptr %self, align 4
  ret void

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hcad0f3a89a1b36aaE(ptr align 1 @str.0, i64 28, ptr align 8 @alloc_9c1fe1ff2e3925050b4dd61043010002) #5
  unreachable
}

; probe5::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe55probe17he8718216f8412b27E() unnamed_addr #1 {
start:
  %x = alloca i32, align 4
  store i32 1, ptr %x, align 4
; call <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
  call void @"_ZN66_$LT$i32$u20$as$u20$core..ops..arith..AddAssign$LT$$RF$i32$GT$$GT$10add_assign17h808330a3e4e24bd3E"(ptr align 4 %x, ptr align 4 @alloc_2e38410fced2c310c68bdf2d45d0c3bd)
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #3

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17hcad0f3a89a1b36aaE(ptr align 1, i64, ptr align 8) unnamed_addr #4

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #3 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #4 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #5 = { noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.74.0 (79e9716c9 2023-11-13)"}
!3 = !{}
