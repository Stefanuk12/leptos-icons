use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12 19 7-7 3 3-7 7-3-3z" ></ path > < path d = "m18 13-1.5-7.5L2 2l3.5 14.5L13 18l5-5z" ></ path > < path d = "m2 2 7.586 7.586" ></ path > < circle r = "2" cx = "11" cy = "11" ></ circle > < / > } } pub const LUCIDE_PEN_TOOL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;