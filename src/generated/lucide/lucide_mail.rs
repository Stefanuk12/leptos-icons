use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" x = "2" y = "4" rx = "2" height = "16" ></ rect > < path d = "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" ></ path > < / > } } pub const LUCIDE_MAIL : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;