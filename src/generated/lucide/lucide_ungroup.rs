use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "8" height = "6" y = "4" x = "5" rx = "1" ></ rect > < rect height = "6" x = "11" rx = "1" y = "14" width = "8" ></ rect > < / > } } pub const LUCIDE_UNGROUP : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor")] } ;