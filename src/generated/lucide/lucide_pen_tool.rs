use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12 19 7-7 3 3-7 7-3-3z" ></ path > < path d = "m18 13-1.5-7.5L2 2l3.5 14.5L13 18l5-5z" ></ path > < path d = "m2 2 7.586 7.586" ></ path > < circle cx = "11" r = "2" cy = "11" ></ circle > < / > } } pub const LUCIDE_PEN_TOOL : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;