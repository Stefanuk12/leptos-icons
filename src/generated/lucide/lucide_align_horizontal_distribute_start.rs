use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 2v20" ></ path > < path d = "M14 2v20" ></ path > < / > } } pub const LucideAlignHorizontalDistributeStart : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;