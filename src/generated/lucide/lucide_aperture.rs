use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < path d = "m14.31 8 5.74 9.94" ></ path > < path d = "M9.69 8h11.48" ></ path > < path d = "m7.38 12 5.74-9.94" ></ path > < path d = "M9.69 16 3.95 6.06" ></ path > < path d = "M14.31 16H2.83" ></ path > < path d = "m16.62 12-5.74 9.94" ></ path > < / > } } pub const LUCIDE_APERTURE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;