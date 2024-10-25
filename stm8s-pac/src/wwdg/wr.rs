#[doc = "Register `WR` reader"]
pub type R = crate::R<WR_SPEC>;
#[doc = "Register `WR` writer"]
pub type W = crate::W<WR_SPEC>;
#[doc = "Field `W0` reader - "]
pub type W0_R = crate::BitReader;
#[doc = "Field `W0` writer - "]
pub type W0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W1` reader - "]
pub type W1_R = crate::BitReader;
#[doc = "Field `W1` writer - "]
pub type W1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W2` reader - "]
pub type W2_R = crate::BitReader;
#[doc = "Field `W2` writer - "]
pub type W2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W3` reader - "]
pub type W3_R = crate::BitReader;
#[doc = "Field `W3` writer - "]
pub type W3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W4` reader - "]
pub type W4_R = crate::BitReader;
#[doc = "Field `W4` writer - "]
pub type W4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W5` reader - "]
pub type W5_R = crate::BitReader;
#[doc = "Field `W5` writer - "]
pub type W5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W6` reader - "]
pub type W6_R = crate::BitReader;
#[doc = "Field `W6` writer - "]
pub type W6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn w0(&self) -> W0_R {
        W0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn w1(&self) -> W1_R {
        W1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn w2(&self) -> W2_R {
        W2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn w3(&self) -> W3_R {
        W3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn w4(&self) -> W4_R {
        W4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn w5(&self) -> W5_R {
        W5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn w6(&self) -> W6_R {
        W6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn w0(&mut self) -> W0_W<WR_SPEC> {
        W0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn w1(&mut self) -> W1_W<WR_SPEC> {
        W1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn w2(&mut self) -> W2_W<WR_SPEC> {
        W2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn w3(&mut self) -> W3_W<WR_SPEC> {
        W3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn w4(&mut self) -> W4_W<WR_SPEC> {
        W4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn w5(&mut self) -> W5_W<WR_SPEC> {
        W5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn w6(&mut self) -> W6_W<WR_SPEC> {
        W6_W::new(self, 6)
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
#[doc = "WWDR window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_SPEC;
impl crate::RegisterSpec for WR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wr::R`](R) reader structure"]
impl crate::Readable for WR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wr::W`](W) writer structure"]
impl crate::Writable for WR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR to value 0x7f"]
impl crate::Resettable for WR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
