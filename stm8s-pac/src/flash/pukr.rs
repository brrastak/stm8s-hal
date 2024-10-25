#[doc = "Register `PUKR` reader"]
pub type R = crate::R<PUKR_SPEC>;
#[doc = "Register `PUKR` writer"]
pub type W = crate::W<PUKR_SPEC>;
#[doc = "Field `MASS_PRG` reader - "]
pub type MASS_PRG_R = crate::FieldReader;
#[doc = "Field `MASS_PRG` writer - "]
pub type MASS_PRG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mass_prg(&self) -> MASS_PRG_R {
        MASS_PRG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn mass_prg(&mut self) -> MASS_PRG_W<PUKR_SPEC> {
        MASS_PRG_W::new(self, 0)
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
#[doc = "Flash program memory unprotection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pukr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pukr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUKR_SPEC;
impl crate::RegisterSpec for PUKR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pukr::R`](R) reader structure"]
impl crate::Readable for PUKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pukr::W`](W) writer structure"]
impl crate::Writable for PUKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUKR to value 0"]
impl crate::Resettable for PUKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
