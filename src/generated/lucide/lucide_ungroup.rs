use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" x = "5" width = "8" y = "4" rx = "1" ></ rect > < rect width = "8" height = "6" x = "11" rx = "1" y = "14" ></ rect > < / > } } pub const LUCIDE_UNGROUP : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;