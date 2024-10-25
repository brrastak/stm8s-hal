#[doc = "Register `FREQR` reader"]
pub type R = crate::R<FREQR_SPEC>;
#[doc = "Register `FREQR` writer"]
pub type W = crate::W<FREQR_SPEC>;
#[doc = "Field `FREQ` reader - "]
pub type FREQ_R = crate::FieldReader;
#[doc = "Field `FREQ` writer - "]
pub type FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<FREQR_SPEC> {
        FREQ_W::new(self, 0)
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
#[doc = "I2C frequency register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freqr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freqr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FREQR_SPEC;
impl crate::RegisterSpec for FREQR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`freqr::R`](R) reader structure"]
impl crate::Readable for FREQR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`freqr::W`](W) writer structure"]
impl crate::Writable for FREQR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQR to value 0"]
impl crate::Resettable for FREQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
