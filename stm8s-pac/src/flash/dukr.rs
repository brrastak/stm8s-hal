#[doc = "Register `DUKR` reader"]
pub type R = crate::R<DUKR_SPEC>;
#[doc = "Register `DUKR` writer"]
pub type W = crate::W<DUKR_SPEC>;
#[doc = "Field `MASS_DATA` reader - "]
pub type MASS_DATA_R = crate::FieldReader;
#[doc = "Field `MASS_DATA` writer - "]
pub type MASS_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mass_data(&self) -> MASS_DATA_R {
        MASS_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn mass_data(&mut self) -> MASS_DATA_W<DUKR_SPEC> {
        MASS_DATA_W::new(self, 0)
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
#[doc = "Data EEPROM unprotection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dukr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dukr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DUKR_SPEC;
impl crate::RegisterSpec for DUKR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dukr::R`](R) reader structure"]
impl crate::Readable for DUKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dukr::W`](W) writer structure"]
impl crate::Writable for DUKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DUKR to value 0"]
impl crate::Resettable for DUKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
