#[doc = "Register `ITR` reader"]
pub type R = crate::R<ITR_SPEC>;
#[doc = "Register `ITR` writer"]
pub type W = crate::W<ITR_SPEC>;
#[doc = "Field `ITERREN` reader - "]
pub type ITERREN_R = crate::BitReader;
#[doc = "Field `ITERREN` writer - "]
pub type ITERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEVTEN` reader - "]
pub type ITEVTEN_R = crate::BitReader;
#[doc = "Field `ITEVTEN` writer - "]
pub type ITEVTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITBUFEN` reader - "]
pub type ITBUFEN_R = crate::BitReader;
#[doc = "Field `ITBUFEN` writer - "]
pub type ITBUFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn iterren(&self) -> ITERREN_R {
        ITERREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn itevten(&self) -> ITEVTEN_R {
        ITEVTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn itbufen(&self) -> ITBUFEN_R {
        ITBUFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iterren(&mut self) -> ITERREN_W<ITR_SPEC> {
        ITERREN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn itevten(&mut self) -> ITEVTEN_W<ITR_SPEC> {
        ITEVTEN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn itbufen(&mut self) -> ITBUFEN_W<ITR_SPEC> {
        ITBUFEN_W::new(self, 2)
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
#[doc = "I2C interrupt control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITR_SPEC;
impl crate::RegisterSpec for ITR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itr::R`](R) reader structure"]
impl crate::Readable for ITR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itr::W`](W) writer structure"]
impl crate::Writable for ITR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITR to value 0"]
impl crate::Resettable for ITR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
