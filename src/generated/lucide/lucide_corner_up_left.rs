use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "9 14 4 9 9 4" ></ polyline > < path d = "M20 20v-7a4 4 0 0 0-4-4H4" ></ path > < / > } } pub const LucideCornerUpLeft : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;