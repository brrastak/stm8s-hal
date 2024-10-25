#[doc = "Register `CANCCR` reader"]
pub type R = crate::R<CANCCR_SPEC>;
#[doc = "Register `CANCCR` writer"]
pub type W = crate::W<CANCCR_SPEC>;
#[doc = "Field `CANDIV` reader - "]
pub type CANDIV_R = crate::FieldReader;
#[doc = "Field `CANDIV` writer - "]
pub type CANDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn candiv(&self) -> CANDIV_R {
        CANDIV_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn candiv(&mut self) -> CANDIV_W<CANCCR_SPEC> {
        CANDIV_W::new(self, 0)
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
#[doc = "CAN clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`canccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`canccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CANCCR_SPEC;
impl crate::RegisterSpec for CANCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`canccr::R`](R) reader structure"]
impl crate::Readable for CANCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`canccr::W`](W) writer structure"]
impl crate::Writable for CANCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CANCCR to value 0"]
impl crate::Resettable for CANCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
