use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "16" rx = "2" x = "4" width = "6" y = "6" ></ rect > < rect height = "9" width = "6" x = "14" rx = "2" y = "6" ></ rect > < path d = "M22 2H2" ></ path > < / > } } pub const LUCIDE_ALIGN_START_HORIZONTAL : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;