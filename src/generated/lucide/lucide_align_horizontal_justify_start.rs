use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "14" width = "6" y = "5" rx = "2" x = "6" ></ rect > < rect width = "6" x = "16" y = "7" height = "10" rx = "2" ></ rect > < path d = "M2 2v20" ></ path > < / > } } pub const LucideAlignHorizontalJustifyStart : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;