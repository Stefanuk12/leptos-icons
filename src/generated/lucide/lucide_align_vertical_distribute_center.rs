use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 7h-5" ></ path > < path d = "M7 7H1" ></ path > < path d = "M22 17h-3" ></ path > < path d = "M5 17H2" ></ path > < / > } } pub const LucideAlignVerticalDistributeCenter : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;