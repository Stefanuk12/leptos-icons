use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "7" width = "18" x = "3" y = "3" rx = "1" ></ rect > < rect y = "14" height = "7" rx = "1" x = "3" width = "9" ></ rect > < rect width = "5" x = "16" rx = "1" height = "7" y = "14" ></ rect > < / > } } pub const LUCIDE_LAYOUT_TEMPLATE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("fill" , "none")] } ;