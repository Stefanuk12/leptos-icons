use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" width = "8" x = "5" y = "4" height = "6" ></ rect > < rect height = "6" width = "8" y = "14" rx = "1" x = "11" ></ rect > < / > } } pub const LUCIDE_UNGROUP : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24")] } ;