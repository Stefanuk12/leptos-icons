use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12 8-9.04 9.06a2.82 2.82 0 1 0 3.98 3.98L16 12" ></ path > < circle cx = "17" r = "5" cy = "7" ></ circle > < / > } } pub const LUCIDE_MIC_VOCAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none")] } ;