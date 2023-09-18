                use std::io::*;
            fn main(){let mut t= 0f32
         ;while t<10.{let(us,uc)=(t*1.3
       ).sin_cos();let(ts,tc)=t.sin_cos()
      ;let(mut zb,mut b)=([1e2;1600],[ 32;
    1644]) ;b[0]=27;b[1]=91;b[2]=50;b[3]=74;
   for i in 0..40{b[4+i*41]=10 }for a in  0..
  9801{let(θ,φ)=((a% 99) as f32*0.0634,(a/99)
  as f32*0.0634);let       ((θs,θc),(φs,φc))=
 (θ .sin_cos( ),φ.           sin_cos() );let (
 mut x,mut y,mut               z)=((1.+0.4*θs)
 *φc,(1.+0.4*θs)               *φs,0.4*θc);let
(mut nx,mut ny,                 mut nz)=(θs*φc,
 θs*φs,θc);(y,z)               =(y*tc+z*ts,z *
 tc-y*ts);(ny,nz               )=(ny*tc+nz*ts,
 nz*tc-ny*ts);(x,z           )=(x*uc+z*us,3.+z
  *uc-x*us);(nx,nz)=       (nx*uc+nz*us,nz*uc
  -nx*us);let d=x*x+y*y+z*z;let(y,x)=((-y/z *
   18.+20.5) as usize,(x/z*30.+20.5)as usize
    );if d<zb[y*40+x]{zb[y*40+x]=d;b[4+y*41
      +x]=b".,-=~:;!*#$@"[(((0.4*nx+0.3 *
       ny-0.7*nz).clamp(0.,1.)*11.)+0.5)
         as usize];}}stdout().write(&b
            );t+=1e-3;}}///////////
                ///////////////
