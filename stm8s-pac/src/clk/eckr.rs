#[doc = "Register `ECKR` reader"]
pub type R = crate::R<ECKR_SPEC>;
#[doc = "Register `ECKR` writer"]
pub type W = crate::W<ECKR_SPEC>;
#[doc = "Field `HSEEN` reader - "]
pub type HSEEN_R = crate::BitReader;
#[doc = "Field `HSEEN` writer - "]
pub type HSEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - "]
pub type HSERDY_R = crate::BitReader;
#[doc = "Field `HSERDY` writer - "]
pub type HSERDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hseen(&self) -> HSEEN_R {
        HSEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hseen(&mut self) -> HSEEN_W<ECKR_SPEC> {
        HSEEN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn hserdy(&mut self) -> HSERDY_W<ECKR_SPEC> {
        HSERDY_W::new(self, 1)
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
#[doc = "External clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECKR_SPEC;
impl crate::RegisterSpec for ECKR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eckr::R`](R) reader structure"]
impl crate::Readable for ECKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eckr::W`](W) writer structure"]
impl crate::Writable for ECKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECKR to value 0"]
impl crate::Resettable for ECKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
