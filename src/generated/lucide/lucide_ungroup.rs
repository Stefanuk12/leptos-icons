use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" y = "4" rx = "1" width = "8" x = "5" ></ rect > < rect width = "8" height = "6" y = "14" rx = "1" x = "11" ></ rect > < / > } } pub const LUCIDE_UNGROUP : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;