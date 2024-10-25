#[doc = "Register `RXCRCR` reader"]
pub type R = crate::R<RXCRCR_SPEC>;
#[doc = "Register `RXCRCR` writer"]
pub type W = crate::W<RXCRCR_SPEC>;
#[doc = "Field `RXCRC` reader - "]
pub type RXCRC_R = crate::FieldReader;
#[doc = "Field `RXCRC` writer - "]
pub type RXCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rxcrc(&mut self) -> RXCRC_W<RXCRCR_SPEC> {
        RXCRC_W::new(self, 0)
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
#[doc = "SPI Rx CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCRCR_SPEC;
impl crate::RegisterSpec for RXCRCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxcrcr::R`](R) reader structure"]
impl crate::Readable for RXCRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxcrcr::W`](W) writer structure"]
impl crate::Writable for RXCRCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCRCR to value 0xff"]
impl crate::Resettable for RXCRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
