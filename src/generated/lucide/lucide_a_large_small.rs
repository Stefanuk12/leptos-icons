use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 14h-5" ></ path > < path d = "M16 16v-3.5a2.5 2.5 0 0 1 5 0V16" ></ path > < path d = "M4.5 13h6" ></ path > < path d = "m3 16 4.5-9 4.5 9" ></ path > < / > } } pub const LucideALargeSmall : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;