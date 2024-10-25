#[doc = "Register `BKR` reader"]
pub type R = crate::R<BKR_SPEC>;
#[doc = "Register `BKR` writer"]
pub type W = crate::W<BKR_SPEC>;
#[doc = "Field `LOCK` reader - "]
pub type LOCK_R = crate::FieldReader;
#[doc = "Field `LOCK` writer - "]
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSSI` reader - "]
pub type OSSI_R = crate::BitReader;
#[doc = "Field `OSSI` writer - "]
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSR` reader - "]
pub type OSSR_R = crate::BitReader;
#[doc = "Field `OSSR` writer - "]
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKE` reader - "]
pub type BKE_R = crate::BitReader;
#[doc = "Field `BKE` writer - "]
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - "]
pub type BKP_R = crate::BitReader;
#[doc = "Field `BKP` writer - "]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOE` reader - "]
pub type AOE_R = crate::BitReader;
#[doc = "Field `AOE` writer - "]
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOE` reader - "]
pub type MOE_R = crate::BitReader;
#[doc = "Field `MOE` writer - "]
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(self.bits & 3)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<BKR_SPEC> {
        LOCK_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<BKR_SPEC> {
        OSSI_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<BKR_SPEC> {
        OSSR_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<BKR_SPEC> {
        BKE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<BKR_SPEC> {
        BKP_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<BKR_SPEC> {
        AOE_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<BKR_SPEC> {
        MOE_W::new(self, 7)
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
#[doc = "TIM1 break register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKR_SPEC;
impl crate::RegisterSpec for BKR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bkr::R`](R) reader structure"]
impl crate::Readable for BKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bkr::W`](W) writer structure"]
impl crate::Writable for BKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKR to value 0"]
impl crate::Resettable for BKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
