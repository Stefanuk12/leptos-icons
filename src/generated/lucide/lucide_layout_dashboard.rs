use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" x = "3" width = "7" y = "3" height = "9" ></ rect > < rect height = "5" rx = "1" width = "7" y = "3" x = "14" ></ rect > < rect width = "7" height = "9" rx = "1" y = "12" x = "14" ></ rect > < rect rx = "1" y = "16" height = "5" x = "3" width = "7" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;