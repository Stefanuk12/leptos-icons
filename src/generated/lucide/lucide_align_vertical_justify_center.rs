use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" x = "5" height = "6" y = "16" rx = "2" ></ rect > < rect y = "2" x = "7" height = "6" width = "10" rx = "2" ></ rect > < path d = "M2 12h20" ></ path > < / > } } pub const LucideAlignVerticalJustifyCenter : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2")] } ;