use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "5" rx = "1" width = "8" height = "6" y = "4" ></ rect > < rect height = "6" x = "11" y = "14" width = "8" rx = "1" ></ rect > < / > } } pub const LUCIDE_UNGROUP : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;