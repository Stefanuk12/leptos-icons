use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2 9 3-3 3 3" ></ path > < path d = "M13 18H7a2 2 0 0 1-2-2V6" ></ path > < path d = "m22 15-3 3-3-3" ></ path > < path d = "M11 6h6a2 2 0 0 1 2 2v10" ></ path > < / > } } pub const LUCIDE_REPEAT_2 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;