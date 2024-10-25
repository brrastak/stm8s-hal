#[doc = "Register `CKDIVR` reader"]
pub type R = crate::R<CKDIVR_SPEC>;
#[doc = "Register `CKDIVR` writer"]
pub type W = crate::W<CKDIVR_SPEC>;
#[doc = "Field `CPUDIV` reader - "]
pub type CPUDIV_R = crate::FieldReader;
#[doc = "Field `CPUDIV` writer - "]
pub type CPUDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HSIDIV` reader - "]
pub type HSIDIV_R = crate::FieldReader;
#[doc = "Field `HSIDIV` writer - "]
pub type HSIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new((self.bits >> 3) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn cpudiv(&mut self) -> CPUDIV_W<CKDIVR_SPEC> {
        CPUDIV_W::new(self, 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn hsidiv(&mut self) -> HSIDIV_W<CKDIVR_SPEC> {
        HSIDIV_W::new(self, 3)
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
#[doc = "Clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckdivr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckdivr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKDIVR_SPEC;
impl crate::RegisterSpec for CKDIVR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ckdivr::R`](R) reader structure"]
impl crate::Readable for CKDIVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ckdivr::W`](W) writer structure"]
impl crate::Writable for CKDIVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKDIVR to value 0x18"]
impl crate::Resettable for CKDIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
