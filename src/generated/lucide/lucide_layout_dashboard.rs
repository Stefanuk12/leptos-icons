use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" x = "3" height = "9" y = "3" width = "7" ></ rect > < rect width = "7" height = "5" y = "3" x = "14" rx = "1" ></ rect > < rect rx = "1" x = "14" height = "9" y = "12" width = "7" ></ rect > < rect width = "7" rx = "1" y = "16" height = "5" x = "3" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;