#[doc = "Register `FPR` reader"]
pub type R = crate::R<FPR_SPEC>;
#[doc = "Register `FPR` writer"]
pub type W = crate::W<FPR_SPEC>;
#[doc = "Field `WPB0` reader - "]
pub type WPB0_R = crate::BitReader;
#[doc = "Field `WPB0` writer - "]
pub type WPB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPB1` reader - "]
pub type WPB1_R = crate::BitReader;
#[doc = "Field `WPB1` writer - "]
pub type WPB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPB2` reader - "]
pub type WPB2_R = crate::BitReader;
#[doc = "Field `WPB2` writer - "]
pub type WPB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPB3` reader - "]
pub type WPB3_R = crate::BitReader;
#[doc = "Field `WPB3` writer - "]
pub type WPB3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPB4` reader - "]
pub type WPB4_R = crate::BitReader;
#[doc = "Field `WPB4` writer - "]
pub type WPB4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPB5` reader - "]
pub type WPB5_R = crate::BitReader;
#[doc = "Field `WPB5` writer - "]
pub type WPB5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wpb0(&self) -> WPB0_R {
        WPB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wpb1(&self) -> WPB1_R {
        WPB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wpb2(&self) -> WPB2_R {
        WPB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wpb3(&self) -> WPB3_R {
        WPB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wpb4(&self) -> WPB4_R {
        WPB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wpb5(&self) -> WPB5_R {
        WPB5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wpb0(&mut self) -> WPB0_W<FPR_SPEC> {
        WPB0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn wpb1(&mut self) -> WPB1_W<FPR_SPEC> {
        WPB1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn wpb2(&mut self) -> WPB2_W<FPR_SPEC> {
        WPB2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wpb3(&mut self) -> WPB3_W<FPR_SPEC> {
        WPB3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn wpb4(&mut self) -> WPB4_W<FPR_SPEC> {
        WPB4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn wpb5(&mut self) -> WPB5_W<FPR_SPEC> {
        WPB5_W::new(self, 5)
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
#[doc = "Flash protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPR_SPEC;
impl crate::RegisterSpec for FPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fpr::R`](R) reader structure"]
impl crate::Readable for FPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpr::W`](W) writer structure"]
impl crate::Writable for FPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPR to value 0"]
impl crate::Resettable for FPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
