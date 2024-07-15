use core::arch::asm;
use crate::types::*;

/* 
SBIの仕様に沿ってOpenSBIを呼び出すための関数
呼び出し規約は以下を参考
ref：https://github.com/riscv-non-isa/riscv-sbi-doc
すべての SBI 関数は、単一のバイナリ・エンコーディングを共有するため、SBI 拡張の混在が容易になります。SBI 仕様では、以下の呼び出し規約に従っています。
- ECALLは、スーパーバイザとSEE間の制御転送命令として使用される。
- a7 は、SBI 拡張 ID（EID）を符号化する
- a6 は、SBI v0.2 以降に定義された SBI 拡張について、a7 でエンコードされた所定の拡張 ID の SBI 機能 ID（FID）をエンコードする。
- a0とa1を除くすべてのレジスタは、着呼側によってSBI呼び出しの間保持されなければならない。
    →a2からa7までのレジスタの値は呼び出し後もそのままであることが保証される
- SBI関数は、a0とa1の値のペアを返す必要があり、a0はエラーコードを返す。これは、C言語の構造体を使用して返される。
*/
#[no_mangle]
pub fn sbi_call(arg0: isize, arg1: isize, arg2: isize, arg3: isize, arg4: isize, arg5: isize, fid: isize, eid: isize) -> SbiRet {
    let error: isize;
    let value: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            options(nostack)
        );
    }
    SbiRet { error, value }
}

/* 
Open SBIのConsole Putchar関数を呼び出すための関数
chに存在するデータをデバッグコンソールに書き込む
sbi_console_getchar()とは異なり、このSBIコールは、送信すべき保留中の文字が残っている場合、
または受信端末がまだバイトを受信する準備ができていない場合、ブロックされる。
しかし、コンソールが全く存在しない場合、その文字は捨てられる。
*/
#[no_mangle]
pub fn putchar(ch: Uint8T) {
    sbi_call(ch as isize, 0, 0, 0, 0, 0, 0, 1);
}

#[no_mangle]
pub unsafe fn memset(buf: *mut Uint8T, c: Uint8T, n: SizeT) -> *mut u8 {
    let mut p = buf;
    for _ in 0..n {
        *p = c;
        p = p.add(1);
    }
    buf
}