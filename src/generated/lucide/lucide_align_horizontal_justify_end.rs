use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "5" width = "6" height = "14" x = "2" ></ rect > < rect rx = "2" height = "10" y = "7" width = "6" x = "12" ></ rect > < path d = "M22 2v20" ></ path > < / > } } pub const LucideAlignHorizontalJustifyEnd : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;