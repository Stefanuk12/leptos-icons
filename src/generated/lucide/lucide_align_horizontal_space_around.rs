use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "7" rx = "2" height = "10" x = "9" width = "6" ></ rect > < path d = "M4 22V2" ></ path > < path d = "M20 22V2" ></ path > < / > } } pub const LucideAlignHorizontalSpaceAround : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;