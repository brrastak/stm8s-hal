#[doc = "Register `DB7RH` reader"]
pub type R = crate::R<DB7RH_SPEC>;
#[doc = "Register `DB7RH` writer"]
pub type W = crate::W<DB7RH_SPEC>;
#[doc = "Field `DBH` reader - "]
pub type DBH_R = crate::FieldReader;
#[doc = "Field `DBH` writer - "]
pub type DBH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dbh(&self) -> DBH_R {
        DBH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dbh(&mut self) -> DBH_W<DB7RH_SPEC> {
        DBH_W::new(self, 0)
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
#[doc = "ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db7rh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db7rh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DB7RH_SPEC;
impl crate::RegisterSpec for DB7RH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`db7rh::R`](R) reader structure"]
impl crate::Readable for DB7RH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`db7rh::W`](W) writer structure"]
impl crate::Writable for DB7RH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DB7RH to value 0"]
impl crate::Resettable for DB7RH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
