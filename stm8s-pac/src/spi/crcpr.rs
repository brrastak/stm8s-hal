#[doc = "Register `CRCPR` reader"]
pub type R = crate::R<CRCPR_SPEC>;
#[doc = "Register `CRCPR` writer"]
pub type W = crate::W<CRCPR_SPEC>;
#[doc = "Field `CRCPOLY` reader - "]
pub type CRCPOLY_R = crate::FieldReader;
#[doc = "Field `CRCPOLY` writer - "]
pub type CRCPOLY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<CRCPR_SPEC> {
        CRCPOLY_W::new(self, 0)
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
#[doc = "SPI CRC polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCPR_SPEC;
impl crate::RegisterSpec for CRCPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crcpr::R`](R) reader structure"]
impl crate::Readable for CRCPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcpr::W`](W) writer structure"]
impl crate::Writable for CRCPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCPR to value 0x07"]
impl crate::Resettable for CRCPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
