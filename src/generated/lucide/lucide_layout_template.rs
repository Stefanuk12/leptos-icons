use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "7" x = "3" rx = "1" width = "18" ></ rect > < rect width = "9" rx = "1" height = "7" x = "3" y = "14" ></ rect > < rect rx = "1" width = "5" height = "7" x = "16" y = "14" ></ rect > < / > } } pub const LUCIDE_LAYOUT_TEMPLATE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24")] } ;