use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "14" rx = "2" x = "4" y = "5" width = "6" ></ rect > < rect rx = "2" y = "7" height = "10" x = "14" width = "6" ></ rect > < path d = "M17 22v-5" ></ path > < path d = "M17 7V2" ></ path > < path d = "M7 22v-3" ></ path > < path d = "M7 5V2" ></ path > < / > } } pub const LucideAlignHorizontalDistributeCenter : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;