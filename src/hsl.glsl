//Source: https://www.shadertoy.com/view/wt23Rt

//Hue to RGB (red, green, blue).
//Source: https://github.com/tobspr/GLSL-Color-Spaces/blob/master/ColorSpaces.inc.glsl
#ifndef saturate
#define saturate(v) clamp(v,0.,1.)
//      clamp(v,0.,1.)
#endif
vec3 hue2rgb(float hue){
	hue=fract(hue);
	return saturate(vec3(
		abs(hue*6.-3.)-1.,
		2.-abs(hue*6.-2.),
		2.-abs(hue*6.-4.)
	));
}

//HSV (hue, saturation, value) to RGB.
//Sources: https://gist.github.com/yiwenl/745bfea7f04c456e0101, https://gist.github.com/sugi-cho/6a01cae436acddd72bdf
vec3 hsv2rgb(vec3 c){
	vec4 K=vec4(1.,2./3.,1./3.,3.);
	return c.z*mix(K.xxx,saturate(abs(fract(c.x+K.xyz)*6.-K.w)-K.x),c.y);
}

//RGB to HSV.
//Source: https://gist.github.com/yiwenl/745bfea7f04c456e0101
vec3 rgb2hsv(vec3 c) {
	float cMax=max(max(c.r,c.g),c.b),
	      cMin=min(min(c.r,c.g),c.b),
	      delta=cMax-cMin;
	vec3 hsv=vec3(0.,0.,cMax);
	if(cMax>cMin){
		hsv.y=delta/cMax;
		if(c.r==cMax){
			hsv.x=(c.g-c.b)/delta;
		}else if(c.g==cMax){
			hsv.x=2.+(c.b-c.r)/delta;
		}else{
			hsv.x=4.+(c.r-c.g)/delta;
		}
		hsv.x=fract(hsv.x/6.);
	}
	return hsv;
}
//Source: https://gist.github.com/sugi-cho/6a01cae436acddd72bdf
vec3 rgb2hsv_2(vec3 c){
	vec4 K=vec4(0.,-1./3.,2./3.,-1.),
	     p=mix(vec4(c.bg ,K.wz),vec4(c.gb,K.xy ),step(c.b,c.g)),
	     q=mix(vec4(p.xyw,c.r ),vec4(c.r ,p.yzx),step(p.x,c.r));
	float d=q.x-min(q.w,q.y),
	      e=1e-10;
	return vec3(abs(q.z+(q.w-q.y)/(6.*d+e)),d/(q.x+e),q.x);
}

//RGB to HSL (hue, saturation, lightness/luminance).
//Source: https://gist.github.com/yiwenl/745bfea7f04c456e0101
vec3 rgb2hsl(vec3 c){
	float cMin=min(min(c.r,c.g),c.b),
	      cMax=max(max(c.r,c.g),c.b),
	      delta=cMax-cMin;
	vec3 hsl=vec3(0.,0.,(cMax+cMin)/2.);
	if(delta!=0.0){ //If it has chroma and isn't gray.
		if(hsl.z<.5){
			hsl.y=delta/(cMax+cMin); //Saturation.
		}else{
			hsl.y=delta/(2.-cMax-cMin); //Saturation.
		}
		float deltaR=(((cMax-c.r)/6.)+(delta/2.))/delta,
		      deltaG=(((cMax-c.g)/6.)+(delta/2.))/delta,
		      deltaB=(((cMax-c.b)/6.)+(delta/2.))/delta;
		//Hue.
		if(c.r==cMax){
			hsl.x=deltaB-deltaG;
		}else if(c.g==cMax){
			hsl.x=(1./3.)+deltaR-deltaB;
		}else{ //if(c.b==cMax){
			hsl.x=(2./3.)+deltaG-deltaR;
		}
		hsl.x=fract(hsl.x);
	}
	return hsl;
}

//HSL to RGB.
//Source: https://github.com/Jam3/glsl-hsl2rgb/blob/master/index.glsl
/*float hueRamp(float a,float b,float hue){
	hue=fract(hue);
	float o=a;
	if((6.*hue)<1.){
		o=a+(b-a)*6.*hue;
	}else if((2.*hue)<1.){
		o=b;
	}else if((3.*hue)<2.){
		o=a+(b-a)*((2./3.)-hue)*6.;
	}
	return o;
}*/
vec3 hsl2rgb(vec3 hsl){
	if(hsl.y==0.){
		return vec3(hsl.z); //Luminance.
	}else{
		float b;
		if(hsl.z<.5){
			b=hsl.z*(1.+hsl.y);
		}else{
			b=hsl.z+hsl.y-hsl.y*hsl.z;
		}
		float a=2.*hsl.z-b;
		return a+hue2rgb(hsl.x)*(b-a);
		/*vec3(
			hueRamp(a,b,hsl.x+(1./3.)),
			hueRamp(a,b,hsl.x),
			hueRamp(a,b,hsl.x-(1./3.))
		);*/
	}
}

//RGB to YCbCr, ranges [0, 1].
//Source: https://github.com/tobspr/GLSL-Color-Spaces/blob/master/ColorSpaces.inc.glsl
vec3 rgb2ycbcr(vec3 c){
	float y=.299*c.r+.587*c.g+.114*c.b;
	return vec3(y,(c.b-y)*.565,(c.r-y)*.713);
}

//YCbCr to RGB.
vec3 ycbcr2rgb(vec3 yuv){
	return vec3(
		yuv.x+1.403*yuv.z,
		yuv.x- .344*yuv.y-.714*yuv.z,
		yuv.x+1.770*yuv.y
	);
}

//CIE L*a*b* (CIELAB, L* for lightness, a* from green to red, b* from blue to yellow)
//Source: https://gist.github.com/mattatz/44f081cac87e2f7c8980 (HLSL)
vec3 rgb2xyz(vec3 c){
	vec3 tmp=vec3(
		(c.r>.04045)?pow((c.r+.055)/1.055,2.4):c.r/12.92,
		(c.g>.04045)?pow((c.g+.055)/1.055,2.4):c.g/12.92,
		(c.b>.04045)?pow((c.b+.055)/1.055,2.4):c.b/12.92
	);
	mat3 mat=mat3(
		.4124,.3576,.1805,
		.2126,.7152,.0722,
		.0193,.1192,.9505
	);
	return 100.*(tmp*mat);
}
vec3 xyz2lab(vec3 c){
	vec3 n=c/vec3(95.047,100.,108.883),
	     v=vec3(
		(n.x>.008856)?pow(n.x,1./3.):(7.787*n.x)+(16./116.),
		(n.y>.008856)?pow(n.y,1./3.):(7.787*n.y)+(16./116.),
		(n.z>.008856)?pow(n.z,1./3.):(7.787*n.z)+(16./116.)
	);
	return vec3((116.*v.y)-16.,500.*(v.x-v.y),200.*(v.y-v.z));
}
vec3 rgb2lab(vec3 c){
	vec3 lab=xyz2lab(rgb2xyz(c));
	return vec3(lab.x/100.,.5+.5*(lab.y/127.),.5+.5*(lab.z/127.));
}
vec3 lab2xyz(vec3 c){
	float fy=(c.x+16.)/116.,
	      fx=c.y/500.+fy,
	      fz=fy-c.z/200.;
	return vec3(
		 95.047*((fx>.206897)?fx*fx*fx:(fx-16./116.)/7.787),
		100.   *((fy>.206897)?fy*fy*fy:(fy-16./116.)/7.787),
		108.883*((fz>.206897)?fz*fz*fz:(fz-16./116.)/7.787)
	);
}
vec3 xyz2rgb(vec3 c){
	mat3 mat=mat3(
		3.2406,-1.5372,-.4986,
		-.9689, 1.8758, .0415,
		 .0557, -.2040,1.0570
	);
	vec3 v=(c/100.0)*mat,
	     r=vec3(
		(v.r>.0031308)?((1.055*pow(v.r,(1./2.4)))-.055):12.92*v.r,
		(v.g>.0031308)?((1.055*pow(v.g,(1./2.4)))-.055):12.92*v.g,
		(v.b>.0031308)?((1.055*pow(v.b,(1./2.4)))-.055):12.92*v.b
	);
	return r;
}
vec3 lab2rgb(vec3 c){return xyz2rgb(lab2xyz(vec3(100.*c.x,2.*127.*(c.y-.5),2.*127.*(c.z-.5))));}

//RGB to sRGB (standard Red Green Blue).
//Source: https://github.com/tobspr/GLSL-Color-Spaces/blob/master/ColorSpaces.inc.glsl
const float SRGB_ALPHA=.055;
float linear2srgb(float x){
	if(x<=.0031308){
		return 12.92*x;
	}else{
		return(1.+SRGB_ALPHA)*pow(x,1./2.4)-SRGB_ALPHA;
	}
}
vec3 rgb2srgb(vec3 c){
	return vec3(
		linear2srgb(c.r),
		linear2srgb(c.g),
		linear2srgb(c.b)
	);
}
//sRGB to RGB.
float srgb2linear(float x) {
	if(x<=.04045){
		return x/12.92;
	}else{
		return pow((x+SRGB_ALPHA)/(1.+SRGB_ALPHA),2.4);
	}
}
vec3 srgb2rgb(vec3 c){
	return vec3(
		srgb2linear(c.r),
		srgb2linear(c.g),
		srgb2linear(c.b)
	);
}

//XYZ to CIE 1931 Yxy color space (luma (Y) along with x and y chromaticity), I found that Photoshop used this.
vec3 xyz2yxy(vec3 c){
	float s=c.x+c.y+c.z;
	return vec3(c.y,c.x/s,c.y/s); //Blue's within s.
}
vec3 yxy2xyz(vec3 c){
	float x=c.x*(c.y/c.z); //Y*(x/y)
	return vec3(x,c.x,(x/c.y)-x-c.x); //(X,Y,(X/x)-X-Y)
}
vec3 rgb2yxy(vec3 c){return xyz2yxy(rgb2xyz(c));}
vec3 yxy2rgb(vec3 c){return xyz2rgb(yxy2xyz(c));}

//RGB to CMYK (cyan, magenta, yellow, key).
vec4 rgb2cmyk(vec3 c){
	float k=1.-max(max(c.r,c.g),c.b);
	return vec4(
		(1.-c.r-k)/(1.-k),
		(1.-c.g-k)/(1.-k),
		(1.-c.b-k)/(1.-k),
		k
	);
}
//CMYK to RGB.
vec3 cmyk2rgb(vec4 c){
	return vec3(
		(1.-c.x)*(1.-c.w),
		(1.-c.y)*(1.-c.w),
		(1.-c.z)*(1.-c.w)
	);
}
vec3 inkColors[18]=vec3[]( //SWOP (Coated), RGB and Yxy.
	vec3(0.,1.,1.),   vec3(26.25,.1673,.2328), //C
	vec3(1.,0.,1.),   vec3(14.5,.4845,.2396),  //M
	vec3(1.,1.,0.),   vec3(71.2,.4357,.5013),  //Y
	vec3(1.,0.,0.),   vec3(14.09,.6075,.3191), //MY
	vec3(0.,1.,0.),   vec3(19.25,.2271,.5513), //CY
	vec3(0.,0.,1.),   vec3(2.98,.2052,.1245),  //CM
	vec3(.25,.25,.25),vec3(2.79,.3227,.2962),  //CMY
	vec3(1.,1.,1.),   vec3(83.02,.3149,.3321), //W
	vec3(0.,0.,0.),   vec3(.82,.3202,.3241)    //K
);
//TODO: Look at SWOP 2006 ICC Profile.
//vec3 pal[]=vec3[](vec3(0.,.5765,.8275),vec3(.8,0.05,.4196),vec3(1.,.9451,.0471),vec3(.0784));
vec3 cmyk2rgb_pal(vec4 c){ //This might be an incorrect way of blending.
	/*return saturate((1.-(
		(c.x*(1.-vec3(0.,.5765,.8275)))+
		(c.y*(1.-vec3(.8,0.05,.4196)))+
		(c.z*(1.-vec3(1.,.9451,.0471)))
	))*(1.-(c.w*.9216)));*/
	return
		mix(vec3(1.),vec3(0.,.5765,.8275),c.x)*
		mix(vec3(1.),vec3(.8,0.05,.4196),c.y)*
		mix(vec3(1.),vec3(1.,.9451,.0471),c.z)*
		mix(1.,.0784,c.w);
}

// void mainImage(out vec4 fragColor,in vec2 fragCoord){
//     vec2 uv=fragCoord/iResolution.xy;
//     vec3 col=hsl2rgb(vec3(uv.x,1.,uv.y));
// 	col=cmyk2rgb_pal(rgb2cmyk(col));
//     col=rgb2lab(col);
//     col.g+=cos(iTime+uv.x)*.05;
//     col.b+=sin(iTime+uv.y)*.05;
//     col=lab2rgb(col);
//     fragColor=vec4(col,1.0);
// }
