#[doc = "Register `SPR3` reader"]
pub type R = crate::R<SPR3_SPEC>;
#[doc = "Register `SPR3` writer"]
pub type W = crate::W<SPR3_SPEC>;
#[doc = "Field `VECT8SPR` reader - "]
pub type VECT8SPR_R = crate::FieldReader;
#[doc = "Field `VECT8SPR` writer - "]
pub type VECT8SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT9SPR` reader - "]
pub type VECT9SPR_R = crate::FieldReader;
#[doc = "Field `VECT9SPR` writer - "]
pub type VECT9SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT10SPR` reader - "]
pub type VECT10SPR_R = crate::FieldReader;
#[doc = "Field `VECT10SPR` writer - "]
pub type VECT10SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT11SPR` reader - "]
pub type VECT11SPR_R = crate::FieldReader;
#[doc = "Field `VECT11SPR` writer - "]
pub type VECT11SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vect8spr(&self) -> VECT8SPR_R {
        VECT8SPR_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vect9spr(&self) -> VECT9SPR_R {
        VECT9SPR_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn vect10spr(&self) -> VECT10SPR_R {
        VECT10SPR_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn vect11spr(&self) -> VECT11SPR_R {
        VECT11SPR_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn vect8spr(&mut self) -> VECT8SPR_W<SPR3_SPEC> {
        VECT8SPR_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn vect9spr(&mut self) -> VECT9SPR_W<SPR3_SPEC> {
        VECT9SPR_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn vect10spr(&mut self) -> VECT10SPR_W<SPR3_SPEC> {
        VECT10SPR_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn vect11spr(&mut self) -> VECT11SPR_W<SPR3_SPEC> {
        VECT11SPR_W::new(self, 6)
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
#[doc = "Interrupt software priority register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPR3_SPEC;
impl crate::RegisterSpec for SPR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spr3::R`](R) reader structure"]
impl crate::Readable for SPR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spr3::W`](W) writer structure"]
impl crate::Writable for SPR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPR3 to value 0xff"]
impl crate::Resettable for SPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
