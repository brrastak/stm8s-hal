#[doc = "Register `CCER1` reader"]
pub type R = crate::R<CCER1_SPEC>;
#[doc = "Register `CCER1` writer"]
pub type W = crate::W<CCER1_SPEC>;
#[doc = "Field `CC1E` reader - "]
pub type CC1E_R = crate::BitReader;
#[doc = "Field `CC1E` writer - "]
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` reader - "]
pub type CC1P_R = crate::BitReader;
#[doc = "Field `CC1P` writer - "]
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2E` reader - "]
pub type CC2E_R = crate::BitReader;
#[doc = "Field `CC2E` writer - "]
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - "]
pub type CC2P_R = crate::BitReader;
#[doc = "Field `CC2P` writer - "]
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<CCER1_SPEC> {
        CC1E_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<CCER1_SPEC> {
        CC1P_W::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<CCER1_SPEC> {
        CC2E_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<CCER1_SPEC> {
        CC2P_W::new(self, 5)
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
#[doc = "TIM2 capture/compare enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCER1_SPEC;
impl crate::RegisterSpec for CCER1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccer1::R`](R) reader structure"]
impl crate::Readable for CCER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccer1::W`](W) writer structure"]
impl crate::Writable for CCER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCER1 to value 0"]
impl crate::Resettable for CCER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
