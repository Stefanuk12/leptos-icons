use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" rx = "1" width = "7" height = "9" ></ rect > < rect width = "7" y = "3" rx = "1" height = "5" x = "14" ></ rect > < rect y = "12" height = "9" x = "14" width = "7" rx = "1" ></ rect > < rect rx = "1" width = "7" y = "16" x = "3" height = "5" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;