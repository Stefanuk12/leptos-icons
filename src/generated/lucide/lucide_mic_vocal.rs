use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12 8-9.04 9.06a2.82 2.82 0 1 0 3.98 3.98L16 12" ></ path > < circle cy = "7" r = "5" cx = "17" ></ circle > < / > } } pub const LUCIDE_MIC_VOCAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24")] } ;