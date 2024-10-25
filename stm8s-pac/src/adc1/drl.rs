#[doc = "Register `DRL` reader"]
pub type R = crate::R<DRL_SPEC>;
#[doc = "Register `DRL` writer"]
pub type W = crate::W<DRL_SPEC>;
#[doc = "Field `DL` reader - "]
pub type DL_R = crate::FieldReader;
#[doc = "Field `DL` writer - "]
pub type DL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dl(&mut self) -> DL_W<DRL_SPEC> {
        DL_W::new(self, 0)
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
#[doc = "ADC data register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRL_SPEC;
impl crate::RegisterSpec for DRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`drl::R`](R) reader structure"]
impl crate::Readable for DRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`drl::W`](W) writer structure"]
impl crate::Writable for DRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DRL to value 0"]
impl crate::Resettable for DRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
