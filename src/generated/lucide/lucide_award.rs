use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "6" cx = "12" cy = "8" ></ circle > < path d = "M15.477 12.89 17 22l-5-3-5 3 1.523-9.11" ></ path > < / > } } pub const LUCIDE_AWARD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;