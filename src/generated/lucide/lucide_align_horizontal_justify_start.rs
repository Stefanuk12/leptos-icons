use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "14" y = "5" width = "6" rx = "2" x = "6" ></ rect > < rect x = "16" height = "10" y = "7" width = "6" rx = "2" ></ rect > < path d = "M2 2v20" ></ path > < / > } } pub const LucideAlignHorizontalJustifyStart : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;