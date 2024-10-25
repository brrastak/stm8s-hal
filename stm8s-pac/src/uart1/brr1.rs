#[doc = "Register `BRR1` reader"]
pub type R = crate::R<BRR1_SPEC>;
#[doc = "Register `BRR1` writer"]
pub type W = crate::W<BRR1_SPEC>;
#[doc = "Field `UART_DIV` reader - "]
pub type UART_DIV_R = crate::FieldReader;
#[doc = "Field `UART_DIV` writer - "]
pub type UART_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_div(&self) -> UART_DIV_R {
        UART_DIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn uart_div(&mut self) -> UART_DIV_W<BRR1_SPEC> {
        UART_DIV_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART1 baud rate register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRR1_SPEC;
impl crate::RegisterSpec for BRR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`brr1::R`](R) reader structure"]
impl crate::Readable for BRR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brr1::W`](W) writer structure"]
impl crate::Writable for BRR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRR1 to value 0"]
impl crate::Resettable for BRR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
