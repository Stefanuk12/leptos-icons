use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < path d = "m14.31 8 5.74 9.94" ></ path > < path d = "M9.69 8h11.48" ></ path > < path d = "m7.38 12 5.74-9.94" ></ path > < path d = "M9.69 16 3.95 6.06" ></ path > < path d = "M14.31 16H2.83" ></ path > < path d = "m16.62 12-5.74 9.94" ></ path > < / > } } pub const LUCIDE_APERTURE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;