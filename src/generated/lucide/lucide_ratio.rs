use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "12" rx = "2" height = "20" y = "2" x = "6" ></ rect > < rect height = "12" y = "6" rx = "2" x = "2" width = "20" ></ rect > < / > } } pub const LUCIDE_RATIO : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;