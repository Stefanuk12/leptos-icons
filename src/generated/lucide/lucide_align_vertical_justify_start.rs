use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "5" height = "6" width = "14" y = "16" ></ rect > < rect width = "10" height = "6" x = "7" y = "6" rx = "2" ></ rect > < path d = "M2 2h20" ></ path > < / > } } pub const LucideAlignVerticalJustifyStart : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;