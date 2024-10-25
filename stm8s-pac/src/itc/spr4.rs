#[doc = "Register `SPR4` reader"]
pub type R = crate::R<SPR4_SPEC>;
#[doc = "Register `SPR4` writer"]
pub type W = crate::W<SPR4_SPEC>;
#[doc = "Field `VECT12SPR` reader - "]
pub type VECT12SPR_R = crate::FieldReader;
#[doc = "Field `VECT12SPR` writer - "]
pub type VECT12SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT13SPR` reader - "]
pub type VECT13SPR_R = crate::FieldReader;
#[doc = "Field `VECT13SPR` writer - "]
pub type VECT13SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT14SPR` reader - "]
pub type VECT14SPR_R = crate::FieldReader;
#[doc = "Field `VECT14SPR` writer - "]
pub type VECT14SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT15SPR` reader - "]
pub type VECT15SPR_R = crate::FieldReader;
#[doc = "Field `VECT15SPR` writer - "]
pub type VECT15SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vect12spr(&self) -> VECT12SPR_R {
        VECT12SPR_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vect13spr(&self) -> VECT13SPR_R {
        VECT13SPR_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn vect14spr(&self) -> VECT14SPR_R {
        VECT14SPR_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn vect15spr(&self) -> VECT15SPR_R {
        VECT15SPR_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn vect12spr(&mut self) -> VECT12SPR_W<SPR4_SPEC> {
        VECT12SPR_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn vect13spr(&mut self) -> VECT13SPR_W<SPR4_SPEC> {
        VECT13SPR_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn vect14spr(&mut self) -> VECT14SPR_W<SPR4_SPEC> {
        VECT14SPR_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn vect15spr(&mut self) -> VECT15SPR_W<SPR4_SPEC> {
        VECT15SPR_W::new(self, 6)
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
#[doc = "Interrupt software priority register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPR4_SPEC;
impl crate::RegisterSpec for SPR4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spr4::R`](R) reader structure"]
impl crate::Readable for SPR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spr4::W`](W) writer structure"]
impl crate::Writable for SPR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPR4 to value 0xff"]
impl crate::Resettable for SPR4_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
